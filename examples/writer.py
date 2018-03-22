import fcsv


def write_list():
    writer = fcsv.writer('v.csv')
    writer.writerow(("Hello", "こんにちは", "csv"))
    writer.writerows((
        ("Hello1", "こんにちは1", "csv1"),
        ("World2", "せかい2", "csv2"),
    ))


def write_tuple():
    writer = fcsv.writer('v.csv')
    writer.writerow(["Hello", "こんにちは", "csv"])
    writer.writerows([
        ["Hello1", "こんにちは1", "csv1"],
        ("World2", "せかい2", "csv2"),
    ])


write_list()
write_tuple()
