# my_search
Goal: Get personalized search results while having privacy.

## How it works
Request will be sent to localhost, `my_search` will then get results from duckduckgo.com or any other online and offline sources, merge them and send the result back to the browser.

## Run
    $ git clone https://github.com/ansvonwa/my_search.git
    $ cargo run

## TODO

- [x] provide results from duckduckgo
- [ ] implement custom !bangs
- [ ] security (send private results only to localhost)
- [ ] matches from bookmarks and personal history
- [ ] merge results together
- [ ] generate local results for files


