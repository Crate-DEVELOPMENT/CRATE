import { useState, useEffect } from 'react';
import { DragDropContext, Droppable } from 'react-beautiful-dnd';
import { WorkspaceCard } from '@/components/WorkspaceCard';
import { AIWorkflowBuilder } from '@/components/AIWorkflowBuilder';
import { Navigation } from '@/components/Navigation';
import { useWallet } from '@solana/wallet-adapter-react';
import { useConnection } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

interface Workspace {
  id: string;
  name: string;
  description: string;
  apps: string[];
  lastModified: Date;
}

export default function Home() {
  const { connection } = useConnection();
  const { connected, publicKey } = useWallet();
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (connected && publicKey) {
      fetchWorkspaces();
    }
  }, [connected, publicKey]);

  const fetchWorkspaces = async () => {
    try {
      // In a real app, this would fetch from your API
      const mockWorkspaces = [
        {
          id: '1',
          name: 'DeFi Trading',
          description: 'Automated trading workspace',
          apps: ['Jupiter', 'Orca', 'Raydium'],
          lastModified: new Date()
        },
        {
          id: '2',
          name: 'NFT Management',
          description: 'NFT tracking and trading',
          apps: ['Magic Eden', 'Tensor', 'Cardinal'],
          lastModified: new Date()
        }
      ];
      setWorkspaces(mockWorkspaces);
    } catch (error) {
      console.error('Error fetching workspaces:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDragEnd = (result: any) => {
    if (!result.destination) return;

    const items = Array.from(workspaces);
    const [reorderedItem] = items.splice(result.source.index, 1);
    items.splice(result.destination.index, 0, reorderedItem);

    setWorkspaces(items);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-black">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="flex justify-between items-center mb-12">
          <h1 className="text-4xl font-bold text-white">
            Your AI Workspace
          </h1>
          <WalletMultiButton />
        </div>

        {!connected ? (
          <div className="text-center py-20 bg-gray-800/50 rounded-xl">
            <h2 className="text-2xl text-gray-200 mb-6">
              Connect your wallet to access your workspaces
            </h2>
            <p className="text-gray-400 mb-8">
              Create automated workflows and manage your DeFi operations with AI assistance
            </p>
          </div>
        ) : (
          <DragDropContext onDragEnd={handleDragEnd}>
            <div className="grid grid-cols-1 gap-6">
              <div className="bg-gray-800/50 p-6 rounded-xl mb-8">
                <AIWorkflowBuilder />
              </div>
              
              <Droppable droppableId="workspaces">
                {(provided) => (
                  <div
                    {...provided.droppableProps}
                    ref={provided.innerRef}
                    className="space-y-4"
                  >
                    {workspaces.map((workspace, index) => (
                      <WorkspaceCard
                        key={workspace.id}
                        workspace={workspace}
                        index={index}
                      />
                    ))}
                    {provided.placeholder}
                  </div>
                )}
              </Droppable>
            </div>
          </DragDropContext>
        )}
      </div>
    </div>
  );
}
