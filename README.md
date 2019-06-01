# Hedera JavaScript SDK


[![](https://img.shields.io/badge/project-Excalibur__-ef5777.svg?style=popout-square)](https://github.com/xclbrio)
[![GitHub](https://img.shields.io/github/license/xclbrio/hedera-sdk-javascript.svg?style=flat-square)](https://github.com/xclbrio/hedera-sdk-javascript/blob/master/LICENSE)
[![Travis (.com)](https://img.shields.io/travis/com/xclbrio/hedera-sdk-javascript.svg?style=flat-square)](https://travis-ci.com/xclbrio/hedera-sdk-javascript)
[![GitHub release](https://img.shields.io/github/release/xclbrio/hedera-sdk-javascript.svg?style=flat-square)](https://github.com/xclbrio/hedera-sdk-javascript/releases)
[![Gitter](https://img.shields.io/gitter/room/:user/:repo.svg?style=flat-square)](https://gitter.im/xclbrio/Lobby)
[![Telegram Chat](https://img.shields.io/badge/chat-Telegram-blue.svg?style=popout-square)](https://t.me/hashgraphdev_ru)

Hedera JavaScript API is the environment for working with Hedera Hashgraph - a consensus platform using the hashgraph consensus algorithm for fast, fair and secure transactions. This is an unofficial version of the development tool by [Excalibur_](https://github.com/xclbrio) team.


## Table of Contents

* [Installation](#installation)
  * [NPM](#npm-not-available-now)
  * [Yarn](#yarn-not-available-now)
* [Usage information](#usage-information)
  * [Import class for use in your project](#import-class-for-use-in-your-project)
  * [Create a class instance](#create-a-class-instance)
  * [Get methods for working with it](#get-methods-for-working-with-it)
  * [An example of a method call](#an-example-of-a-method-call)
* [Other](#other)
  * [Community](#сommunity)
  * [Other implementations](#other-implementations)
  * [More information by Hedera Hashgraph](#more-information-by-hedera-hashgraph)
* [License](#license)

## Installation

### NPM (NOT available now)

```bash
npm install hederasdk
```

### Yarn (NOT available now)

```bash
yarn add hederasdk
```

## Usage information

### Import class for use in your project

Connection HederaSDK library for further use:

```js
const HederaSDK = require("hederasdk");
```

### Create a class instance

Creating an instance of the HederaSDK class:

```js
let hederaHashgraph = new HederaSDK("_address_", "_port_", "_targetOS_");
```

### Get methods for working with it

You can get methods for working with your object:

```js
console.log(hederaHashgraph);
```

### An example of a method call

Example of a method call on an object:

```js
hederaHashgraph.version();
```

## Other

### Community
 * Excalibur_ chat: [Gitter](https://gitter.im/xclbrio/Lobby)
 * Email: support@xclbr.io
 * [Issues page](https://github.com/xclbrio/DLL/issues) for reports
 
### Other implementations

Offiсial:
 * [Java](https://github.com/hashgraph/hedera-sdk-java)
 
By launchbadge:
 * [Python](https://github.com/launchbadge/hedera-sdk-python)
 * [Rust](https://github.com/launchbadge/hedera-sdk-rust)
 * [Golang](https://github.com/launchbadge/hedera-sdk-go)
 
 ### More information by Hedera Hashgraph
You can visit [The Hedera Site](https://www.hedera.com/).

Please review the [Contributing Guide](https://github.com/hashgraph/hedera-sdk-java/blob/master/CONTRIBUTING.md).
