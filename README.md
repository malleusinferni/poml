# Poml: Painless Octothorpe Markup Language

**Poml** is a lightweight tree-structured data format loosely inspired by [Org](http://orgmode.org). The original implementation was in Perl in 2012. This is an in-progress rewrite in Rust, intended for use as a serialization format for arbitrary data structures.

It looks like this:

```poml
#!poml v1
#parent
The "contents" of a tag can be any text, comprising any number of lines,
as long as none of them start with an octothorpe ("#")
##child 1
This child has the attribute string "1"
##child 2
This child has the attribute string "2"
#parent
This tag begins a new subtree in the root of the document. The previous
"parent" tag contains two children, but this one is childless.
```

