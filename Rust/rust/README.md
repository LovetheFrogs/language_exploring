This command shows number of lines in each file and its total. It also sorts the result in ascending order.
```git ls-files | grep '.rs' | xargs wc -l | sort -n``` 
