Building:
	docker build -t nginx .

Running:
	docker run --name nginx -p 8080:80 nginx

```

Go server runtime.GOMAXPROCS(8):
================================================================================
---- Global Information --------------------------------------------------------
> request count                                      89200 (OK=76332  KO=12868 )
> min response time                                      0 (OK=0      KO=34    )
> max response time                                  80441 (OK=58853  KO=80441 )
> mean response time                                 20041 (OK=18582  KO=28696 )
> std deviation                                      18571 (OK=18429  KO=16985 )
> response time 50th percentile                      18405 (OK=16493  KO=30382 )
> response time 75th percentile                      34352 (OK=32249  KO=42132 )
> response time 95th percentile                      51417 (OK=50825  KO=54284 )
> response time 99th percentile                      57772 (OK=57161  KO=69102 )
> mean requests/sec                                405.455 (OK=346.964 KO=58.491)
---- Response Time Distribution ------------------------------------------------
> t < 800 ms                                         27960 ( 31%)
> 800 ms < t < 1200 ms                                1489 (  2%)
> t > 1200 ms                                        46883 ( 53%)
> failed                                             12868 ( 14%)
---- Errors --------------------------------------------------------------------
> i.n.c.AbstractChannel$AnnotatedSocketException: Cannot assign   11827 (91.91%)
requested address: localhost/127.0.0.1:8080
> j.i.IOException: Premature close                                 1041 ( 8.09%)
================================================================================

Go server runtime.GOMAXPROCS(1):
================================================================================
---- Global Information --------------------------------------------------------
> request count                                      89200 (OK=76769  KO=12431 )
> min response time                                      0 (OK=0      KO=121   )
> max response time                                  84511 (OK=65055  KO=84511 )
> mean response time                                 20489 (OK=19066  KO=29275 )
> std deviation                                      19136 (OK=19023  KO=17411 )
> response time 50th percentile                      18355 (OK=16639  KO=31313 )
> response time 75th percentile                      35616 (OK=33331  KO=42716 )
> response time 95th percentile                      54872 (OK=54160  KO=57257 )
> response time 99th percentile                      60904 (OK=60377  KO=68813 )
> mean requests/sec                                 403.62 (OK=347.371 KO=56.249)
---- Response Time Distribution ------------------------------------------------
> t < 800 ms                                         26903 ( 30%)
> 800 ms < t < 1200 ms                                1565 (  2%)
> t > 1200 ms                                        48301 ( 54%)
> failed                                             12431 ( 14%)
---- Errors --------------------------------------------------------------------
> i.n.c.AbstractChannel$AnnotatedSocketException: Cannot assign   11695 (94.08%)
requested address: localhost/127.0.0.1:8080
> j.i.IOException: Premature close                                  736 ( 5.92%)
================================================================================

Nginx static serving worker=1:
================================================================================
---- Global Information --------------------------------------------------------
> request count                                      89200 (OK=78634  KO=10566 )
> min response time                                      0 (OK=0      KO=1341  )
> max response time                                  71329 (OK=52913  KO=71329 )
> mean response time                                 17884 (OK=16728  KO=26486 )
> std deviation                                      15940 (OK=15651  KO=15432 )
> response time 50th percentile                      16851 (OK=15459  KO=22932 )
> response time 75th percentile                      30767 (OK=29919  KO=40143 )
> response time 95th percentile                      44903 (OK=43784  KO=48817 )
> response time 99th percentile                      49801 (OK=49146  KO=63615 )
> mean requests/sec                                422.749 (OK=372.673 KO=50.076)
---- Response Time Distribution ------------------------------------------------
> t < 800 ms                                         22571 ( 25%)
> 800 ms < t < 1200 ms                                1334 (  1%)
> t > 1200 ms                                        54729 ( 61%)
> failed                                             10566 ( 12%)
---- Errors --------------------------------------------------------------------
> i.n.c.AbstractChannel$AnnotatedSocketException: Cannot assign    9263 (87.67%)
requested address: localhost/127.0.0.1:8080
> j.i.IOException: Premature close                                 1303 (12.33%)
================================================================================
```

