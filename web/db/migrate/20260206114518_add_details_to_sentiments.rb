class AddDetailsToSentiments < ActiveRecord::Migration[7.1]
  def change
    # เช็คว่ามีคอลัมน์ source หรือยัง? ถ้ายังไม่มีค่อยสร้าง
    unless column_exists?(:sentiments, :source)
      add_column :sentiments, :source, :string
    end

    # เช็คว่ามีคอลัมน์ recorded_at หรือยัง?
    unless column_exists?(:sentiments, :recorded_at)
      add_column :sentiments, :recorded_at, :datetime
    end
  end
end