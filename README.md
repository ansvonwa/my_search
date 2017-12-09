# my_search
Goal: Get personalized search results and cool features while having privacy.

## How it works
Request will be sent to localhost, `my_search` will then get results from duckduckgo.com and/or any other online and offline sources, merge them and send the result back to the browser.

## Run
    $ git clone https://github.com/ansvonwa/my_search.git
    $ cd my_search
    $ cargo run

## Install
make sure my_search runs always:
* Linux: e.g. add `/path/to/my_search &> /dev/null &` to `.profile`
* Windows: put `my_search.exe` in `%AppData%\Microsoft\Windows\Start Menu\Programs\Startup` folder.

### Firefox
Go to [ready.to](http://ready.to/search/en/?sna=my_search&prf=http%3A%2F%2Flocalhost%3A8080%2F%3Fq%3D&in=utf&ou=ono&mod=pn) and click `OpenSearch plug-in my_search` button. Check box to set as default and confirm.

### Chrome
Go to [settings](chrome://settings/searchEngines), click add and enter "my_search", nothing, and "http://localhost:8080/?q=%s" and click add. Then set as default.

## TODO

- [x] provide results from duckduckgo
- [x] proof of concept for custom !bangs (!yt!)
- [ ] implement config for custom !bangs
- [ ] security (send private results only to localhost)
- [ ] matches from bookmarks and personal history
- [ ] merge results together
- [ ] generate local results for files


