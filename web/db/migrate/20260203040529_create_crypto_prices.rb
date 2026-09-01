class CreateCryptoPrices < ActiveRecord::Migration[8.1]
  def change
    create_table :crypto_prices do |t|
      t.string :symbol
      t.decimal :price
      t.decimal :sentiment_score

      t.timestamps
    end
  end
end
