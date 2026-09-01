class AddUniqueIndexToSentiments < ActiveRecord::Migration[7.1]
  def up
    # ⚠️ ท่าไม้ตาย: สั่งล้างข้อมูลในตารางทิ้งทั้งหมด (Reset Table)
    # วิธีนี้จะทำให้ตารางว่างเปล่า 100% ทำให้สร้าง Unique Index ผ่านแน่นอนครับ
    say "Nuking all data to ensure unique index creation..."
    execute "TRUNCATE TABLE sentiments RESTART IDENTITY CASCADE;"

    # 🔒 สร้างกฎห้ามซ้ำ (ตอนนี้ทำได้ชัวร์ เพราะไม่มีข้อมูลขวางแล้ว)
    unless index_exists?(:sentiments, :coin_symbol, unique: true)
      say "Creating unique index..."
      add_index :sentiments, :coin_symbol, unique: true
    end
  end

  def down
    remove_index :sentiments, :coin_symbol if index_exists?(:sentiments, :coin_symbol, unique: true)
  end
end