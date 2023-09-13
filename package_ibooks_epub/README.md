## 打包 iBooks 中的 epub 图书文件

一个用于重新打包 iBooks 中的 epub 图书文件的工具。

场景：
epub 图书在导入到 iBooks 之后，会被解压并添加 plist 文件。
如果使用 calibre 导入 iBooks 中的 epub 文件夹，会显示为文件夹而无法导入。
使用此工具能够将 iBooks 中的 epub 文件夹重新打包为可被 calibre 读取的 epub 文件。

用法：

```bash
./package_ibooks_epub <存储了 iBooks 图书文件的目录> <重新打包后保存的目录>
# 比如下面的指令能将 iBooks 书库中的图书重新打包并保存到“下载”目录中
./package_ibooks_epub /Users/${USER}/Library/Mobile\ Documents/iCloud\~com\~apple\~iBooks/Documents/ /Users/${USER}/Downloads
```
