# Reflection - Module 08:

### 1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?
**Jawab:**
* **Unary**: Klien mengirim satu *request* kepada server dan menerima tepat satu *response*. Mekanisme ini cocok digunakan untuk operasi pengambilan data tunggal (seperti profil pengguna), autentikasi, atau melakukan satu kalkulasi yang hasilnya langsung dikembalikan.
* **Server Streaming**: Klien mengirim satu *request*, lalu server merespons dengan aliran (*stream*) data secara berurutan hingga tidak ada lagi pesan yang tersisa. Cocok untuk mengirimkan pembaruan secara kontinu atau data yang sangat besar seperti harga saham *real-time*, *news feed*, atau pengiriman *file* besar secara bertahap.
* **Bi-directional Streaming**: Klien dan server saling mengirim aliran pesan secara dua arah dan independen dalam koneksi yang sama. Cocok untuk komunikasi interaktif berkelanjutan, seperti aplikasi *chatting* *real-time* atau analitik *real-time* di mana pertukaran data terjadi dari kedua sisi tanpa henti.

### 2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?
**Jawab:**
Pertimbangan keamanan saat mengimplementasikan gRPC (terutama di Rust) mencakup *Data Encryption*, yaitu memastikan seluruh komunikasi dienkripsi menggunakan protokol TLS (Transport Layer Security) di atas HTTP/2 agar data biner tidak dapat disadap di tengah jalan. *Authentication* (Autentikasi) diperlukan untuk memverifikasi identitas klien yang memanggil service gRPC, yang biasanya dilakukan dengan menyisipkan JWT (JSON Web Tokens) pada HTTP headers/metadata atau menggunakan mTLS (Mutual TLS). Sedangkan *Authorization* (Otorisasi) harus diterapkan pada level kode/logika bisnis atau melalui *interceptor* untuk memastikan klien yang sudah terautentikasi memiliki hak akses (izin) yang sah untuk mengeksekusi metode RPC tertentu.

### 3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?
**Jawab:**
Beberapa tantangan dalam menangani *bidirectional streaming* di Rust antara lain adalah manajemen memori dan koneksi, karena koneksi yang bertahan lama (*long-lived connection*) berpotensi memicu kebocoran memori jika aliran data tidak ditutup dengan benar saat klien terputus. Tantangan lainnya adalah penanganan konkurensi (seperti menggunakan blok asynchronous Tokio), karena kita harus memastikan sinkronisasi antara proses membaca pesan dari klien dan menulis pesan balasan tidak menyebabkan *deadlock* atau *race conditions*. Selain itu, mengelola *backpressure* (agar server tidak kewalahan saat klien mengirim pesan secara sporadis) dan menangani *error* secara halus juga menjadi isu penting.

### 4. What are the advantages and disadvantages of using `tokio_stream::wrappers::ReceiverStream` for streaming responses in Rust gRPC services?
**Jawab:**
* **Keuntungan**: `ReceiverStream` memudahkan konversi sisi penerima dari *channel* Tokio (`tokio::sync::mpsc::Receiver`) menjadi *stream* yang kompatibel dengan standar gRPC/Tonic. Hal ini membuat pengiriman respons asinkron menjadi sangat efisien, karena data yang diproses di *background task* dapat langsung dialirkan sebagai respons gRPC.
* **Kekurangan**: Karena berbasis *channel* MPSC, pendekatan ini membutuhkan alokasi memori tambahan untuk *buffer*. Jika nilai kapasitas *buffer* tidak diatur dengan bijak (terlalu kecil memblokir *sender*, terlalu besar memakan RAM), performa sistem bisa terganggu. Selain itu, manajemen *channel* menambah sedikit *overhead* dibandingkan metode langsung.

### 5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?
**Jawab:**
Struktur kode gRPC di Rust dapat dibuat modular dengan memisahkan *concern* ke dalam beberapa *module*. Kode *protobuf* hasil *generate* diisolasi pada satu modul khusus. Implementasi *service traits* (logika spesifik RPC) harus dipisahkan dari logika bisnis utama. Dengan begitu, logika bisnis utama dapat digunakan ulang apabila aplikasi ingin mengekspos API lain (seperti REST atau GraphQL). Selain itu, komponen seperti koneksi database harus di-*passing* secara eksplisit (misalnya melalui `Arc` di dalam *struct service*), agar *unit testing* lebih mudah dilakukan.

### 6. In the `MyPaymentService` implementation, what additional steps might be necessary to handle more complex payment processing logic?
**Jawab:**
Untuk logika pembayaran yang lebih kompleks, diperlukan langkah-langkah seperti:
1. Validasi *payload request* (format ID valid dan nominal positif).
2. Interaksi dengan *database* asinkron untuk mencatat riwayat transaksi.
3. Integrasi dengan API eksternal (*Payment Gateway*).
4. Penerapan mekanisme *idempotency* untuk menghindari pembayaran ganda.
5. Penanganan kesalahan komprehensif untuk mengirimkan status gRPC yang akurat (seperti `INVALID_ARGUMENT` atau `INTERNAL`).

### 7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?
**Jawab:**
Adopsi gRPC memfasilitasi komunikasi antar-layanan (*service-to-service*) yang lebih terstruktur dan berkinerja tinggi. gRPC mendefinisikan *interface* komunikasi secara ketat menggunakan *Protocol Buffers* dan mendukung pembuatan *client libraries* multi-bahasa, sehingga mempermudah sistem lintas teknologi untuk saling terhubung. Namun, karena gRPC tidak didukung secara native oleh browser, arsitektur biasanya memerlukan *proxy/gateway* (seperti gRPC-Web) jika ingin melayani klien *frontend* web secara langsung.

### 8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?
**Jawab:**
* **Keuntungan HTTP/2**: Mendukung *multiplexing*, memungkinkan pengiriman banyak *request* paralel melalui satu koneksi TCP tanpa *head-of-line blocking*. Memiliki fitur *header compression* untuk memperkecil *overhead*, dan *server push*.
* **Kekurangan**: Kompleksitas implementasi lebih tinggi dibandingkan HTTP/1.1 yang berbasis teks sederhana. Dibandingkan WebSocket, fitur *streaming* gRPC di HTTP/2 lebih sulit dikonsumsi secara langsung di ekosistem *browser* konvensional tanpa alat tambahan.

### 9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?
**Jawab:**
Model *request-response* REST mengharuskan klien membuat koneksi/request baru setiap kali menginginkan data, yang sering mengandalkan *polling* tidak efisien untuk fitur *real-time*. Sebaliknya, *bidirectional streaming* gRPC menggunakan satu koneksi HTTP/2 terbuka di mana klien dan server bisa mengirim data secara independen kapan pun dibutuhkan. Ini menghasilkan komunikasi yang instan, responsif, dan hemat *bandwidth* karena terbebas dari *overhead* pembuatan koneksi berulang.

### 10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?
**Jawab:**
Pendekatan berbasis skema (Protobuf) mewajibkan kontrak data yang ketat. Implikasinya, pesan di-*encode* menjadi format biner yang padat dan cepat di-*parsing*, serta integrasi menjadi *type-safe*. Sebaliknya, JSON bersifat fleksibel dan mudah dibaca manusia, namun memerlukan validasi data manual yang lebih intensif serta mengonsumsi *bandwidth* dan waktu pemrosesan yang lebih besar dibandingkan format biner Protobuf.