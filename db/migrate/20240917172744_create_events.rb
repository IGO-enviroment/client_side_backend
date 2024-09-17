class CreateEvents < ActiveRecord::Migration[7.2]
  def change
    create_table :events do |t|
      t.references :area
      t.references :event_type

      t.string :title, null: false
      t.text :description

      t.datetime :start_at

      t.integer :tickets, default: 0, null: false

      t.boolean :publish, default: false

      t.timestamps
    end
  end
end
