import { useMemo } from 'react';
import type { AppProps } from 'next/app';
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react';
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
  BackpackWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Navigation } from '@/components/Navigation';
import { clusterApiUrl } from '@solana/web3.js';

// Styles
import '@solana/wallet-adapter-react-ui/styles.css';
import '@/styles/globals.css';

// Initialize QueryClient
const queryClient = new QueryClient();

export default function App({ Component, pageProps }: AppProps) {
  // You can also provide a custom RPC endpoint
  const endpoint = useMemo(() => clusterApiUrl('mainnet-beta'), []);

  // Initialize wallet adapters
  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter(),
      new SolflareWalletAdapter(),
      new BackpackWalletAdapter(),
    ],
    []
  );

  return (
    <QueryClientProvider client={queryClient}>
      <ConnectionProvider endpoint={endpoint}>
        <WalletProvider wallets={wallets} autoConnect>
          <WalletModalProvider>
            <div className="min-h-screen bg-gray-900">
              <Navigation />
              <Component {...pageProps} />
            </div>
          </WalletModalProvider>
        </WalletProvider>
      </ConnectionProvider>
    </QueryClientProvider>
  );
}
