class CreateSentiments < ActiveRecord::Migration[7.1]
  def up
    # 1. สร้างตารางเก็บข้อมูล
    create_table :sentiments, id: false do |t|
      t.string :coin_symbol, null: false
      t.decimal :score, precision: 5, scale: 2
      t.decimal :price, precision: 20, scale: 8
      t.string :source
      t.datetime :recorded_at, null: false
    end

    # 2. เปิดใช้งาน TimescaleDB Extension (ถ้ายังไม่ได้เปิด)
    #execute "CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;"

    # 3. แปลงตารางให้เป็น Hypertable (หากฐานข้อมูลรองรับ TimescaleDB)
    begin
      execute "SELECT create_hypertable('sentiments', 'recorded_at', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);"
    rescue => e
      # บน Render เป็น Standard PostgreSQL ธรรมดา (ไม่มี TimescaleDB) ให้ข้ามไปได้โดยไม่ Error
      puts "⚠️ TimescaleDB hypertable skipped: #{e.message}"
    end
    
    # 4. สร้าง Index ให้ค้นหาเร็วๆ
    add_index :sentiments, [:coin_symbol, :recorded_at]
  end

  def down
    drop_table :sentiments
  end
end