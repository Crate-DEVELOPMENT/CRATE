import { useState } from 'react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { 
  Bars3Icon, 
  XMarkIcon,
  SparklesIcon,
  Square3Stack3DIcon,
  ChartBarIcon,
  Cog6ToothIcon
} from '@heroicons/react/24/outline';

export const Navigation = () => {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const router = useRouter();

  const navigation = [
    { name: 'Workspaces', href: '/workspaces', icon: Square3Stack3DIcon },
    { name: 'Analytics', href: '/analytics', icon: ChartBarIcon },
    { name: 'Settings', href: '/settings', icon: Cog6ToothIcon },
  ];

  const isActive = (path: string) => router.pathname === path;

  return (
    <nav className="bg-gray-900/50 backdrop-blur-md border-b border-gray-800">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          {/* Logo and Desktop Navigation */}
          <div className="flex items-center">
            <Link href="/" className="flex items-center">
              <SparklesIcon className="h-8 w-8 text-purple-500" />
              <span className="ml-2 text-xl font-bold text-white">Crate</span>
            </Link>

            <div className="hidden md:flex ml-10 space-x-8">
              {navigation.map((item) => (
                <Link
                  key={item.name}
                  href={item.href}
                  className={`flex items-center px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                    isActive(item.href)
                      ? 'bg-purple-500/10 text-purple-400'
                      : 'text-gray-300 hover:text-white hover:bg-gray-800'
                  }`}
                >
                  <item.icon className="h-5 w-5 mr-2" />
                  {item.name}
                </Link>
              ))}
            </div>
          </div>

          {/* Wallet and Mobile Menu Button */}
          <div className="flex items-center">
            <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700 
                                        !rounded-lg !py-2 !px-4 !text-sm" />
            
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="ml-4 md:hidden p-2 rounded-lg text-gray-400 
                         hover:text-white hover:bg-gray-800"
            >
              {isMobileMenuOpen ? (
                <XMarkIcon className="h-6 w-6" />
              ) : (
                <Bars3Icon className="h-6 w-6" />
              )}
            </button>
          </div>
        </div>
      </div>

      {/* Mobile Navigation */}
      <div className={`md:hidden ${isMobileMenuOpen ? 'block' : 'hidden'}`}>
        <div className="px-2 pt-2 pb-3 space-y-1">
          {navigation.map((item) => (
            <Link
              key={item.name}
              href={item.href}
              className={`flex items-center px-3 py-2 rounded-lg text-base font-medium ${
                isActive(item.href)
                  ? 'bg-purple-500/10 text-purple-400'
                  : 'text-gray-300 hover:text-white hover:bg-gray-800'
              }`}
              onClick={() => setIsMobileMenuOpen(false)}
            >
              <item.icon className="h-5 w-5 mr-2" />
              {item.name}
            </Link>
          ))}
        </div>
      </div>
    </nav>
  );
};
