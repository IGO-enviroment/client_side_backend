class CreateAreas < ActiveRecord::Migration[7.2]
  def change
    create_table :areas do |t|
      t.string :title, null: false

      t.text :description

      t.boolean :publish, default: false
      
      t.text :address_value

      t.timestamps
    end
  end
end
