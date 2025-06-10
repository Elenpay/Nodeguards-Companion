# Nodeguard's Companion

The Nodeguard-Signer-Extension is a browser extension that allows you to load your existing Bitcoin wallets and sign operations by generating a PSBT.


## 🧩 Compatibility
The Signer can be imported into Chrome and Firefox
## 🏗 Installation
### Prerequirements
To build the extension locally you need to have the following tools installed:

* [Docker](https://www.docker.com/)
* [Just](https://github.com/casey/just)
* [1Password CLI](https://developer.1password.com/docs/cli/)

### Building the extension

Clone this repository:
```sh
git clone https://github.com/Elenpay/Nodeguard-Signer-Extension.git
```

To install the dependencies you will have to run:
```sh
just install-depenencies
```
This will build the necessary docker containers for building the extension.

Next, you need to select the right target (manifest v2, v3) from the justfile according to the usage given to the extension.  
For example, we will use the following v2 target:
```sh
just build-extension-v2
```
After running this command you should now have a `dist` folder with all the built artifacts, ready to be imported from a browser

### Serving the extension for development

To serve the extension for development you need to run:
```sh
just serve-extension
```
This will serve the extension on port 9000, you can change the por in the docker compose file.

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


## 🔏 Signing the Extension
To sign the extension for Google Chrome, go to the [Dev Console](https://chrome.google.com/webstore/devconsole) and upload it to the chrome web store.

To sign the extension for Firefox, follow these steps:

1. Ensure you have the necessary API credentials stored in 1Password for signing the extension. These credentials should be stored securely under the item named "Firefox Extension Signing Credentials".
   - **Fields:**
     - `username`: Your API username.
     - `credential`: Your API secret key.
2. Use the `sign-extension` recipe in the `justfile` to sign the extension:
   ```sh
   just sign-extension
   ```
   This will retrieve credentials and sign the extension.
4. Verify that the extension has been signed successfully by checking the output logs or the signed artifacts.


# 🤝 Contributing

  If you'd like to contribute to the Nodeguard-Remote-Signer, please fork
  the repository and submit a pull request with your changes. We welcome
  contributions of all kinds, including bug fixes, new features, and
  documentation improvements.  

# Attributions
<a href="https://www.flaticon.com/free-icons/feather" title="feather icons">Feather icons created by Freepik - Flaticon</a>

# 🔐 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0)

In short, the AGPL-3.0 license allows you to use, copy, modify, and distribute this software for any purpose, as long as any modifications or derivative works are also licensed under the AGPL-3.0 license and any distributed versions of the software include the original source code or a written offer to obtain the source code. However, if you use the software to provide a network service, you must also make the source code available to users of the service under the terms of the AGPL-3.0 license.

For more information about the AGPL-3.0 license, please see [https://www.gnu.org/licenses/agpl-3.0.en.html](https://www.gnu.org/licenses/agpl-3.0.en.html).



