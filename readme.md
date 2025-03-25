# crsgen
Biz/BrowserHT、SD向けのCRSのひな形を作成するツール。引数にCRSクラス名を与えると、与えた順に上からオブジェクトが配置されたCRSスクリプトを標準出力に出力する。

# 例
```
~$ crsgen Label TextBox Button

Form Form1 {
        Width = 390;
        Height = 800;

        Function err(e) {
                MessageBox(e.Message, e.Method + "-" + str(e.Code));
        }

        Label Label1 {
                X = 10;
                Y = 10;
                Width = ^.Width - 20;
                Height = 30;

        }
        TextBox TextBox1 {
                X = 10;
                Y = 50;
                Width = ^.Width - 20;
                Height = 30;

        }
        Button Button1 {
                X = 10;
                Y = 90;
                Width = ^.Width - 20;
                Height = 30;

                Function OnTouch() {}
        }

        if (!$DESIGNTIME) {

        }
}

```

# 解説
オブジェクトのWidth、Heightは固定。FormのWidth=390はiPhone12の画面幅。FormのHeightは適当。

`Function err(e)`はエラーでアプリが続行不能になるのを防ぎながらエラーメッセージを閲覧するための関数。catchの中で使う。

```
try {
	/* ... */
} catch (e) {
	err(e);
}
```