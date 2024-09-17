class CreateVisits < ActiveRecord::Migration[7.2]
  def change
    create_table :visits do |t|
      t.references :model, polymorphic: true

      t.string :ip

      t.bigint :entered

      t.timestamps
    end
  end
end
