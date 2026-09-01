require 'json'
require 'ostruct'

if defined?(Rails::Server)
  Thread.new do
    Rails.application.executor.wrap do
      puts "🎧 Rails Real-time Listener: Initializing..."
      sleep 2 # รอให้ Rails และ Database เตรียมตัวเสร็จสมบูรณ์
      
      loop do
        conn = nil
        begin
          conn = ActiveRecord::Base.connection_pool.checkout
          raw_conn = conn.raw_connection
          raw_conn.exec("LISTEN new_sentiment")
          puts "🎧 Rails Real-time Listener: Connected & Listening!"
          
          loop do
            raw_conn.wait_for_notify do |channel, pid, payload|
              # แปลงข้อมูลจาก Rust
              data = JSON.parse(payload)
              
              sentiment_obj = OpenStruct.new(
                coin_symbol: data['coin_symbol'],
                score: data['score'],
                price: data['price'],
                change_24h: data['change_24h'] || 0.0,
                volume: data['volume'] || 0.0,
                recorded_at: data['recorded_at'] ? Time.parse(data['recorded_at']) : Time.now
              )

              # 1. Update Card (การ์ดด้านบน)
              Turbo::StreamsChannel.broadcast_replace_to(
                "sentiments",
                target: "sentiment_#{data['coin_symbol']}",
                partial: "dashboard/sentiment",
                locals: { sentiment: sentiment_obj }
              )

              # 2. Update Table Row (ตารางด้านล่าง)
              Turbo::StreamsChannel.broadcast_replace_to(
                "feed", 
                target: "row_#{data['coin_symbol']}", 
                partial: "dashboard/feed_item",
                locals: { sentiment: sentiment_obj }
              )
            end
          end
        rescue => e
          puts "❌ Listener Error: #{e.message}. Retrying in 5 seconds..."
          sleep 5
        ensure
          ActiveRecord::Base.connection_pool.checkin(conn) if conn
        end
      end
    end
  end
end