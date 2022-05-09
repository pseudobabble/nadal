# Nadal

`It serves very fast`

Transformers based model serving with Actix.

```
~/code/rust/nadal$ http post :8080 question="why did the chicken cross the road" context="the chicken crossed the road because it was super cool"
HTTP/1.1 200 OK
content-length: 294
date: Mon, 09 May 2022 08:31:52 GMT

Question: "why did the chicken cross the road",
 Context: "the chicken crossed the road because it was super cool",
 Answer [
    [
        Answer {
            score: 0.5398329496383667,
            start: 37,
            end: 54,
            answer: "it was super cool",
        },
    ],
],
```

