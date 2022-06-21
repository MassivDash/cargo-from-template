# Cargo from_template

<div style="display: flex; justify-content: flex-start; gap: 10px; width: 100%; margin: auto; max-width: 1150px;">
  <a href="" target="_blank">
    <img
      alt="Version"
      src="https://img.shields.io/badge/version--blue.svg?cacheSeconds=2592000"
    />
  </a>
  <a href="" target="_blank">
    <img
      alt="Documentation"
      src="https://img.shields.io/badge/documentation-yes-brightgreen.svg"
    />
  </a>
  <a href="#" target="_blank">
    <img
      alt="License: MIT"
      src="https://img.shields.io/badge/License-MIT-yellow.svg"
    />
  </a>
  <a href="https://twitter.com/spaceoutPL" target="_blank">
    <img
      alt="Twitter: spaceoutPL"
      src="https://img.shields.io/twitter/follow/spaceoutPL.svg?style=social"
    />
  </a>
</div>

---
### Description

Cargo-from_template is a command line tool for creating a file or folder containing files from predefined user templates in current path. 


### Install
In order to install  on your computer, clone the project then run the following command from the project folder: 

```sh
cargo install --path . 
```

### Usage


If you followed the official rust documentation and cargo / rustc is in scope, navigate to a input folder and run 

```sh
cargo from_template 

```

Thr cli will ask you for the templates path, you will need at least one 
template to run the program. 

### Templates

A templates folder needs to be created on the user system, each sub folder name will be the name of the future template. A template is a file or files placed in a subfolder for quick component creation.

### Config 

```
templates
│
└───name of the template (ex: react)
│   │
│   └───%name%
│       │   %name%.tsx
│       │   %name%.test.tsx
|       |   index.ts
│   
└───name of the template 
    │   readme.md
```

### Files

This cli uses ``` %something% ``` convention to pick up on the variables. Crete a text file you need to, for example index.html, myComponent.svelte, whatever.txt, inside use the convention mentioned above to write whatever you need, you can have as ```%_many_%``` vars as you want, the program will read the contents and ask for each vars through the cli. 

After a using a name inside the file, the cli will also look at the paths for the same vars and will replace it with the value asked through the cli. The Config scenario above assumes that there is a ```%name%``` is used somewhere inside the subfolder files.  

template example 


```js

import React from 'react';

export interface %name%Props {}

const %name%: React.FunctionComponent<%name%Props> = ({ }) => {
    return <>%name%</>;
};

export default %name%;

```

example of multi vars usage in a file.md

```
---
title: %name%
author: %author%
date: __DATE__
excerpt: %excerpt%
category: %category%
---

# New post from %name%
```


You can also use ```__DATE__``` inside the files or file names, for example ```___DATE__--%name%.md```, this will be exchanged date for current date in ```YYYY-MM-DD``` format.

```
templates
│
└───name of the template (ex: post)
│   │
│   └───__DATE__--%name%
│       │   %name%.md

```


### Additional arguments  

#### Reset the templates folder config

```
cargo from_template --reset
```

#### Run the template 
You can also pass the name of your template to the command line,
this will skip the select template cli step. 

example:
```
cargo from_template react
```



### Author

👤 **Lukasz Celitan**

* Website: https://spaceout.pl
* Twitter: [@spaceoutPL](https://twitter.com/spaceoutPL)
* Github: [@MassivDash](https://github.com/MassivDash)
* LinkedIn: [@luke-celitan-99920b168](https://linkedin.com/in/luke-celitan-99920b168)

## Show your support

Give a ⭐️ if this project helped you!

