import time
import fcsv


while True:
    writer = fcsv.Writer('v.csv')
    writer.writerow(("hoge", "ゔぃ", "fuga"))
    writer.writerows((
        ("hoge", "ゔぃ", "fuga"),
        ("v1", "v2", "v9"),
    ))
    # time.sleep(0.1)
