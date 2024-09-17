class CreateTypeEvents < ActiveRecord::Migration[7.2]
  def change
    create_table :type_events do |t|
      t.string :title, null: false

      t.text :description
      t.boolean :publish, default: false

      t.timestamps
    end
  end
end
