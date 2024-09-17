class CreateContents < ActiveRecord::Migration[7.2]
  def change
    create_table :contents do |t|
      t.references :model, polymorphic: true

      t.integer :kind

      t.text :value

      t.integer :order_value, null: false

      t.json :options

      t.timestamps
    end
  end
end
