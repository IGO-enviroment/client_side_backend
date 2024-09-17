source "https://rubygems.org"

# Bundle edge Rails instead: gem "rails", github: "rails/rails", branch: "main"
gem "rails", "~> 7.2.1"
gem "pg", "~> 1.1"
# Use the Puma web server [https://github.com/puma/puma]
gem "puma", ">= 5.0"

gem "tzinfo-data", platforms: %i[ windows jruby ]
gem "config"

# Reduces boot times through caching; required in config/boot.rb
gem "bootsnap", require: false
gem "whenever", require: false

gem "rack-cors"
gem "image_processing", ">= 1.2"

gem "fast_blank", "~> 1.0"
gem "oj", "~> 3.14"

group :development, :test do
  # See https://guides.rubyonrails.org/debugging_rails_applications.html#debugging-with-the-debug-gem
  gem "debug", platforms: %i[ mri windows ], require: "debug/prelude"

  # Static analysis for security vulnerabilities [https://brakemanscanner.org/]
  gem "brakeman", require: false

  # Omakase Ruby styling [https://github.com/rails/rubocop-rails-omakase/]
  gem "rubocop-rails-omakase", require: false
end

gem "sidekiq", "~> 7.3"

gem "bcrypt", "~> 3.1"

gem "chewy", "~> 7.6"
