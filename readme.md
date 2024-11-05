# Türkçe Deyim ve Atasözleri Sorgulama ve JSON Çıktı Alma Uygulaması

Bu proje, TDK (Türk Dil Kurumu) API’lerini kullanarak Türkçe deyim ve atasözlerini sorgulayıp, bu verileri lokal olarak listeleyebilen ve JSON formatında dışa aktarabilen bir yazılım geliştirilmesini kapsamaktadır. Uygulama Rust ile yazılmıştır.

## Özellikler

- **Deyim ve Atasözü Sorgulama**: TDK API’si üzerinden deyim ve atasözleri sorgulaması yapar.
- **Veri Saklama**: Sorgulanan deyim ve atasözlerini lokal olarak saklar.
- **JSON Dışa Aktarma**: Toplanan verileri JSON formatında dışa aktarma seçeneği sunar.
- **Quiz Oluşturma**: Kaydedilen deyim ve atasözlerinden rastgele bir 10 soruluk quiz oluşturur.


## Gereksinimler

- Rust 1.56 veya üstü sürüm.

## Kurulum

Projeyi bilgisayarınıza klonladıktan sonra aşağıdaki komutu kullanarak derleyebilirsiniz:

```bash
cargo build --release
```

## Kullanım

Uygulamayı pv komutuyla çalıştırabilirsiniz. Yardım menüsünü görüntülemek için -h veya --yardim bayrağını kullanın.

```bash
Usage: pv [options]

Options:
    -g, --guncelle      Kayitli deyim/atasozlerini TDK sozlugunden gunceller.
    -s, --sayi          Veritabaninda kayitli deyim/atasozu sayisini gösterir.
    -c, --cikti DOSYA   Deyim/atasozlerini JSON formatinda DOSYA'ya kaydeder.
    -h, --yardim        Yardim menusunu gösterir.
```

## Örnek Kullanımlar

### Kayıtları Güncelleme
TDK API’den yeni deyim ve atasözlerini çekmek için:

```bash
pv -g
```

### Kayıt Sayısını Görüntüleme
Veritabanında kayıtlı deyim/atasözü sayısını öğrenmek için:

```bash
pv -s
```

### JSON Formatında Çıktı Alma
Verileri JSON formatında output.json dosyasına kaydetmek için:

```bash
pv -c output.json
```

## Quiz Özelliği
Uygulama, kayıtlı deyim ve atasözlerinden rastgele 10 soruluk bir quiz oluşturur. Aşağıda örnek bir quiz sorusu yer almaktadır:

```bash
▣ öfkesi kabarmak
● çok kızmak, sakinleşmişken yeniden öfkelenmek, tekrar sinirlenmek.
○ gerektiğinden çok önce veya henüz ortada hiçbir şey yokken hazırlanmaya kalkışmak.
○ taşıt hurdaya çıkmak.
○ direnmek, istememek, inat etmek, sinirlenmek.
```


## Örnek Çıktı Dosyası

Projede bir örnek çıktı dosyası olarak `deyimler_atasozleri.json` yer almaktadır. Bu dosya, JSON formatında deyim ve atasözleri verisinin nasıl göründüğüne dair bir referans niteliğindedir ve kullanıcılara çıktı formatı hakkında bilgi sunar. Eğer uygulamadan alınan sonuçlar üzerinde işlem yapmak istiyorsanız, `deyimler_atasozleri.json` dosyasını inceleyebilirsiniz.
Bu ciktiyi kullanarak kendiniz deyim ve atasözleri verileri üzerinde calisan uygulamalar gelistirebilerisiniz.

### Örnek `deyimler_atasozleri.json` İçeriği

``` json
[
  {
    "id": 1,
    "proverb": "a'dan z'ye (kadar)",
    "meaning": "baştan aşağı, tamamen, tamamıyla, bütünüyle: Evini a'dan z'ye değiştirdi.",
    "proverb_type": "Deyim"
  },
  {
    "id": 2,
    "proverb": "aba altında er yatar",
    "meaning": "giyim kuşam kişiliğe ölçü olamaz.",
    "proverb_type": "Atasözü"
  }
]
```

## Katkıda Bulunun
Bu projeye katkıda bulunmak için lütfen bir pull request gönderin veya bir issue açın.

## Lisans
Bu proje MIT Lisansı altında lisanslanmıştır.
