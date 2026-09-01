FROM ruby:3.2.2

# ติดตั้ง Node.js และ Postgres Client ที่จำเป็น
RUN apt-get update -qq && apt-get install -y nodejs postgresql-client

# กำหนดโฟลเดอร์ทำงาน
WORKDIR /app

# ก๊อปปี้ไฟล์รายการ Library เข้าไปก่อน
COPY Gemfile /app/Gemfile
COPY Gemfile.lock /app/Gemfile.lock

# ติดตั้ง Library
RUN bundle install

# ก๊อปปี้โค้ดทั้งหมดตามเข้าไป
COPY . /app

# อนุญาตให้รันไฟล์ bin/rails ได้
RUN chmod +x bin/rails

# ⚠️ คำสั่งสำคัญ: สั่งสร้าง CSS/JS เตรียมไว้สำหรับ Production
RUN SECRET_KEY_BASE_DUMMY=1 ./bin/rails assets:precompile

# แก้ไขสิทธิ์ไฟล์ bin/rails ให้รันได้แน่นอน (กันเหนียว)
RUN chmod +x bin/rails

# สั่ง prepare db (สร้าง schema + รัน migrate) ก่อน แล้วค่อยเปิด server
CMD ["/bin/sh", "-c", "bin/rails db:prepare && bundle exec rails server -b 0.0.0.0"]