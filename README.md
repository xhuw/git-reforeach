# git-reforeach

recursively find all git directories, filter on a regex and execute the git command

e.g.

```
# run git status in all the lib_ repos in the current directory
git reforeach "^lib_.*" status
```
