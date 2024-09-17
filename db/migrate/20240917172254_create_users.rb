class CreateUsers < ActiveRecord::Migration[7.2]
  def change
    create_table :users do |t|
      t.string :email, null: false, index: true
      t.text :secure_code

      t.timestamps
    end
  end
end
