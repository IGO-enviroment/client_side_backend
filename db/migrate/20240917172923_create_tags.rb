class CreateTags < ActiveRecord::Migration[7.2]
  def change
    create_table :tags do |t|
      t.string :title, null: false

      t.text :description

      t.integer :groum_name

      t.timestamps
    end
  end
end
