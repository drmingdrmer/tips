- Git Internals
	- Git internal objects: create object, tree,
	  https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
	- Git packfiles
	  https://git-scm.com/book/en/v2/Git-Internals-Packfiles
		- examine pack file content:
		  `git verify-pack -v .git/objects/pack/pack-978e03944f5c581011e6998cd0e9e30000905586.idx`
	- Git Refspec
	  https://git-scm.com/book/en/v2/Git-Internals-The-Refspec
	- Git Transfer protocols
	  https://git-scm.com/book/en/v2/Git-Internals-Transfer-Protocols
- Setup proxy for https remote:
  `export http_proxy="http://"; export https_proxy="http://"`
  `git config --global http.proxy http://` `git config --global https.proxy http://`