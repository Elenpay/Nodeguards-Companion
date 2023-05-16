# Nodeguard's Companion

The Nodeguard-Signer-Extension is a browser extension that allows you to load your existing Bitcoin wallets and sign operations by generating a PSBT.


## 🧩 Compatibility
The Signer can be imported into Chrome and Firefox
## 🏗 Installation
### Prerequirements
To build the extension locally you need to have the following tools installed:

* [Rust](https://www.rust-lang.org/tools/install)
* [Just](https://github.com/casey/just)
* [NodeJS](https://nodejs.org/en)
* [WASM-pack](https://rustwasm.github.io/wasm-pack/installer/)
 
Once you have wasm-pack installed and ready to use, make sure the wasm build target is visible to rustup by running
```sh
rustup target list | grep wasm32-unknown-unknown
```
If the output is empty, try adding it manually with the following command:
```
rustup target add wasm32-unknown-unknown
```
\
Additionally, make sure LLVM is in your system by using your specific package manager.

For example, in mac we would run

```sh
brew install llvm
```

And we would add it to the execution PATH with the following command
```sh
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
```

### Building the extension

Clone this repository:
```sh
git clone https://github.com/Elenpay/Nodeguard-Signer-Extension.git
```
Navigate to the cloned directory and install the dependencies. The module uses `yarn` so get it installed as well:
```sh
npm install -g yarn
```

Build the extension:
You need to select the right target from the justfile according to the usage given to the extension.  
For example, for signing operations in REGTEST we will use the following target:
```sh
just build-extension-regtest
```
After running this command you should now have a `dist` folder with all the built artifacts, ready to be imported from a browser 

### Importing the extension
#### Chrome
Open the Extension Management page by navigating to [Extensions](chrome://extensions). Enable Developer Mode by clicking the toggle switch next to "Developer mode". Click the "Load unpacked" button and select the `dist` directory of the cloned repository.
#### Firefox
Open the Add-ons page by navigating to [Addons](about:addons). Click the settings gear icon and select "Debug Add-ons". Click the "Load Temporary Add-on" button and select the `manifest.json` file inside the `dist` directory of the cloned repository.

Once the extension is successfully installed you should be able to open it. Its name is PSBT Signer
## 🚀 Usage

1. As soon as you open the extension it will ask you to set up a password. The Signer will prompt you to enter this password for any future operation
2. Before you can sign transactions you need to import the wallets you want to sign with.  
The signer supports multiple ways of adding wallets: You can use your private keys or your wallet seed. Alternatively you can generate your own seed.  
Once you have decided on how to import a wallet, type a name and hit `save`. You will be prompted for your password.
3. The signer will automatically detect when an operation needs to be signed. If something needs PSBT signature on screen and you open the extension you will see an `Approve PSBT` box with
the transaction information details such as TxId, Operation type and Amount.  
If you want to sign the operation, you simply need to select a wallet from your previously-configured entries and click on `sign`. You will be asked to enter your password once again and the signer will 
add the resulting PSBT to the correspondent text area in your browser. If you are using the Signer with NodeGuard, then it will automatically complete the sign operation as well.



# 🤝 Contributing

  If you'd like to contribute to the Nodeguard-Remote-Signer, please fork
  the repository and submit a pull request with your changes. We welcome
  contributions of all kinds, including bug fixes, new features, and
  documentation improvements.  

 
# 🔐 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0)

In short, the AGPL-3.0 license allows you to use, copy, modify, and distribute this software for any purpose, as long as any modifications or derivative works are also licensed under the AGPL-3.0 license and any distributed versions of the software include the original source code or a written offer to obtain the source code. However, if you use the software to provide a network service, you must also make the source code available to users of the service under the terms of the AGPL-3.0 license.

For more information about the AGPL-3.0 license, please see [https://www.gnu.org/licenses/agpl-3.0.en.html](https://www.gnu.org/licenses/agpl-3.0.en.html).



