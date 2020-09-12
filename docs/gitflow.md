# Starting with gitflow
gitflow is a simple team workflow using git idealized by Vicent Driessen in 2010 on [this post](https://nvie.com/posts/a-successful-git-branching-model/). And even today it's still widely used by many teams and companies.

The intention of this document is to teach you the basic concepts as quickly as possible and right after guide you into practical scenarios

## Sections
- [The theory](#the-theory)
- [Creating the develop branch](#creating-the-develop-branch)
- [Programming a feature](#programming-a-feature)
- [Doing a release](#doing-a-release)
- [Hotfixes](#hotfixes)

---

## The theory
The main concept is that most of a project's development work can be done in parallel. Given that we can divide the project into five branches:

<img src="https://nvie.com/img/git-model@2x.png" width="350"/>

**main or master:** The main branch idea it's to have the project in a stable state. Ready for any one download it and use it without any bugs.

**develop:** The develop branch it's the most updated compared to the main branch. It has new features and very likely a lot of bugs.

**feature/feature_x:** The feature branches comes from the develop branch and they are were developers do their job implementing new features to the project.

**release/vX.X:** The release branch comes from the develop branch. The idea is that when a project it's getting close to some deadline and/or the develop branch has accumulated enough features, it's time make a branch for only working in bugfixes. Once it reaches a stable state and/or a deadline we merge it to the main branch, and the bugfixes back to the develop branch.

**hotfixes:** As in any project, bugs can show up after a release, that's when the hotfix branch comes to action. The hotfix branch comes from the main branch and it's idea is to get everybody working in a bugfix and make the main branch stable again. After that, the bugfix go back to the develop branch.

---

## Creating the develop branch

Git clone your newly created repository, enter his root folder and run:

    $ git branch develop
    $ git push -u origin develop

This will create the develop branch and send it to the remote repository, like Github.

For other developers download the develop branch they should do:

    $ git checkout -b develop origin/develop

---

## Programming a feature

To start working on your feature do:

    $ git checkout -b feature/my_feature develop

After a lot of work and good commits you think it's time to merge it to the develop branch. First lets check if there some new work in the develop branch so if there is you will probably need to fix some conflict in the code.

    $ git checkout develop
    $ git pull

Now lets try to merge:

    $ git checkout feature/my_feature
    $ git merge develop

If there is some conflict go back to the code, try to fixed and commit it and then:

    $ git branch -d feature/my_feature
    $ git push origin develop

---

## Doing a release

Creating the release branch to start with the bugfixes:

    $ git checkout -b release/vX.X develop

After a lot of good work, bugfixes and commits:

    $ git checkout main
    $ git merge release/vX.X
    $ git tag vX.X

Now sending back the changes done in the release branch:

    $ git checkout develop
    $ git merge release/vX.X

And now that the work is finished:

    $ git branch -d release/vX.X
    $ git push origin vX.X
    $ git push origin main
    $ git push origin develop

---

## Hotfixes

The hotfix branch will follow the same flow as the release branch:

    $ git checkout -b hotfix/vX.X.1 main

After some good work and commits:

    $ git checkout main
    $ git merge hotfix/vX.X.1
    $ git tag vX.X.1

Sending back to the release branch:

    $ git checkout develop
    $ git merge hotfix/vX.X.1

And now:

    $ git branch -d hotfix/vX.X.1
    $ git push origin vX.X.1
    $ git push origin main
    $ git push origin develop

---