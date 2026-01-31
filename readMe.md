These are my personal notes while learning Git by practicing commands.


## commit
git commit -m "I have made some changes to files"

git config --global user.email ""
git config --global user.name ""

git status
Shows the state of working directory and staging area

stage or commit 

git reset HEAD~
Moves HEAD one commit back
Keeps file changes in working directory (by default)

## remove 
if you want to delete the file and the same time stage it then use
git rm fileName
git rm -r folderName 
-r (recusive)

git rm -f filenName
completely deletes the file

git rm --cached fileName
Only removes it from staging

## restore delete file 
Discards all local changes and resets working directory + staging to last commit
git reset --hard 

git reset (only reset the stage changes)

## view commit
checking the commit log
git log
git log --oneline

# Branching

master branch or main branch 

git branch fileName
it inherit the current state of the branch