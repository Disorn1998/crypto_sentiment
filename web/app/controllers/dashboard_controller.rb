class DashboardController < ApplicationController
  def index
    # 1. ดึงข้อมูลล่าสุดของแต่ละเหรียญ (BTC, ETH, SOL)
    # เราใช้ SQL Raw Query เพื่อความเร็วและดึงตัวล่าสุดจริงๆ ของแต่ละเหรียญ
    begin
      @sentiments = Sentiment.find_by_sql("
        SELECT DISTINCT ON (coin_symbol) *
        FROM sentiments
        ORDER BY coin_symbol, recorded_at DESC
      ")
    rescue => e
      Rails.logger.error("⚠️ Error fetching sentiments: #{e.message}")
      @sentiments = []
    end

    # 2. กรณีที่ยังไม่มีข้อมูลเลย (เพิ่งเริ่มระบบ) ให้กัน error ไว้
    @sentiments ||= []
  end
end