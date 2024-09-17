class CreateWeights < ActiveRecord::Migration[7.2]
  def change
    create_table :weights do |t|
      t.references :model, polymorphic: true

      t.float :value

      t.string :type

      t.timestamps
    end
  end
end
