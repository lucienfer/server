import { connect, disconnect } from 'starknetkit';

document.getElementById('connectButton').addEventListener('click', async () => {
	try {
		const wallet = await connect();
		console.log('Wallet connecté:', wallet);
	} catch (error) {
		console.error('Erreur de connexion:', error);
	}
});

document.getElementById('disconnectButton').addEventListener('click', async () => {
	try {
		const disco = await disconnect({clearLastWallet: true});
		console.log('Wallet deconnecté:',disco);
	} catch (error) {
		console.error('Erreur de deconnexion:', error);
	}
});
