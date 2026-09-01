🪙 Thai Crypto Sentinel 2026
### **"วิเคราะห์คริปโตแบบ Real-time พร้อมบันทึกความเชื่อมั่นบน Blockchain"**

**Thai Crypto Sentinel 2026** คือแพลตฟอร์มตรวจสอบตลาดคริปโตเคอเรนซีที่ทำงานด้วยความเร็วสูง โดยดึงข้อมูลสดจาก Binance API มาประมวลผลด้วยสถาปัตยกรรมแบบ **Hybrid** ที่รวมจุดเด่นของเทคโนโลยีระดับโลกไว้ด้วยกัน: ความเร็วของ **Rust**, ความลื่นไหลของ **Ruby on Rails 8** และความโปร่งใสของ **Ethereum Blockchain**

<img width="1606" height="1192" alt="Screenshot 2026-03-06 231218" src="https://github.com/user-attachments/assets/e041190c-560e-4c2d-ae48-16fe763446ae" />


---

### 🛠 Tech Stack (เทคโนโลยีที่ใช้)

| ส่วนงาน | เทคโนโลยีที่เลือกใช้ | เหตุผลและหน้าที่ |
| :--- | :--- | :--- |
| **Core Backend** | **Rust 🦀** (Tokio, Sqlx) | ดึงข้อมูลความถี่สูงจาก Binance และคำนวณคะแนน AI ด้วยความเร็วสูงสุด |
| **Web Framework** | **Ruby on Rails 8 💎** | ใช้ Turbo Streams เพื่อผลักข้อมูลราคาให้เด้งขึ้นหน้าจอทันทีโดยไม่ต้อง Refresh |
| **Blockchain** | **Solidity & Ethers.js ⛓️** | เขียน Smart Contract บน Sepolia Testnet สำหรับบันทึกข้อมูลที่ไม่สามารถแก้ไขได้ |
| **Database** | **PostgreSQL 🐘** | เป็นตัวกลางรับ-ส่งสัญญาณ (Event Broker) ระหว่าง Rust และ Rails |
| **Frontend** | **Tailwind CSS** | ดีไซน์หน้าจอแบบ Modern Dark Mode รองรับการใช้งานบนมือถือ |

---

### ✨ ฟีเจอร์เด่น (Key Features)

* ⚡ **Real-time Data:** ติดตามราคาเหรียญ BTC, ETH, SOL, BNB, DOGE และอื่นๆ แบบวินาทีต่อวินาที
* 🧠 **AI Sentiment Score:** ระบบคำนวณคะแนนความเชื่อมั่น (0-100) วิเคราะห์จากแรงซื้อขายและความผันผวนผ่าน Rust Worker
* 🔗 **On-Chain Verification:** เชื่อมต่อกับ **MetaMask** เพื่อบันทึกค่า AI Score ลงบน Blockchain จริงๆ เพื่อความโปร่งใสและตรวจสอบได้จากภายนอก
* 🖱️ **Interactive UX:** กดที่การ์ดเหรียญหรือตารางราคา เพื่อสลับกราฟ TradingView และเปลี่ยนเหรียญที่จะตรวจสอบได้ทันที
* 📉 **Professional Charts:** บูรณาการกราฟเทคนิคัลระดับโลกจาก TradingView ไว้ในหน้าเดียว

---

### 🏗 Architecture (สถาปัตยกรรมระบบ)

ระบบประกอบด้วย 4 ส่วนหลักที่ทำงานประสานงานกัน:
1.  **Crypto-Worker (Rust):** รับหน้าที่ดึงราคาจาก API มาคำนวณคะแนนแล้วบันทึกเข้าฐานข้อมูล
2.  **Crypto-Web (Rails 8):** รับหน้าที่ส่งข้อมูลจากฐานข้อมูลไปแสดงผลบนเบราว์เซอร์ของผู้ใช้ทันทีแบบ Real-time
3.  **DApp Layer (Solidity):** Smart Contract รับหน้าที่จารึกคะแนน AI ลงบนเครือข่ายบล็อกเชน Sepolia
4.  **Crypto_db (PostgreSQL):** ทำหน้าที่เป็นศูนย์กลางการสื่อสารระหว่างทุก Service เข้าด้วยกัน

---

### 🚀 วิธีรันโปรเจกต์ (Local Development)

1.  **Clone Repository:**
    ```bash
    git clone [https://github.com/Disorn1998/crypto_sentiment.git](https://github.com/Disorn1998/crypto_sentiment.git)
    cd crypto_sentiment
    ```
2.  **รันระบบด้วย Docker:**
    ```bash
    docker-compose up --build
    ```
3.  **ตั้งค่าฐานข้อมูล:**
    ```bash
    docker-compose exec web rails db:migrate
    ```
4.  **เตรียม MetaMask:** เชื่อมต่อกระเป๋าเงินของคุณกับเครือข่าย **Sepolia Testnet** เพื่อทดลองใช้ระบบยืนยันข้อมูลบนบล็อกเชน

---

### 🌐 การปรับแต่งเพื่อใช้งานจริง (Deployment)

โปรเจกต์นี้ได้รับการปรับแต่งเป็นพิเศษสำหรับการใช้งานบน **Render Free Tier**:
* **Memory Management:** ปรับแต่งการ Compile ของ Rust ให้ทำงานได้อย่างเสถียรภายใต้ข้อจำกัด RAM 512MB
* **Web3 Integration:** ใช้ ethers.js เชื่อมต่อโดยตรงกับระบบนิเวศของ Ethereum

---


**Developed by [Disorn Suppartum](https://github.com/Disorn1998)**
นักศึกษาคณะวิศวกรรมคอมพิวเตอร์ มหาวิทยาลัยรามคำแหง 

