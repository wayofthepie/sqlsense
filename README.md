# sqlsense
Making sense of sql.

# Graph Foreign Key Relationships
`sqlsense` can output a `dot` file, graphing SQL foreign key relationships.

```
$ sqlsense path/to/sql.sql example.dot
```

You can convert this to a PNG with `dot`.

```
$ dot -Tpng -o example.png example.dot
```
