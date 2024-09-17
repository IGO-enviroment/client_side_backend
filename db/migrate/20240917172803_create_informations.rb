class CreateInformations < ActiveRecord::Migration[7.2]
  def change
    create_table :informations do |t|
      t.references :area

      t.string :title, null: false
      t.text :description

      t.boolean :publish, default: false

      t.datetime :published_at

      t.timestamps
    end
  end
end
