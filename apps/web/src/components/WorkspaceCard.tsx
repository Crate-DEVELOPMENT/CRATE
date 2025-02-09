import { FC, useState } from 'react';
import { Draggable } from 'react-beautiful-dnd';
import { motion } from 'framer-motion';
import { 
  SparklesIcon, 
  ArrowPathIcon,
  EllipsisHorizontalIcon,
  ChartBarIcon
} from '@heroicons/react/24/outline';

interface Workspace {
  id: string;
  name: string;
  description: string;
  apps: {
    id: string;
    name: string;
    icon: string;
  }[];
  stats: {
    totalValue: number;
    performance24h: number;
  };
  automations: number;
  lastActive: string;
}

interface WorkspaceCardProps {
  workspace: Workspace;
  index: number;
}

export const WorkspaceCard: FC<WorkspaceCardProps> = ({ workspace, index }) => {
  const [isRunning, setIsRunning] = useState(false);

  const handleRunAutomation = async () => {
    try {
      setIsRunning(true);
      // AI automation logic would go here
      await new Promise(resolve => setTimeout(resolve, 2000)); // Simulated delay
    } catch (error) {
      console.error('Automation failed:', error);
    } finally {
      setIsRunning(false);
    }
  };

  return (
    <Draggable draggableId={workspace.id} index={index}>
      {(provided) => (
        <motion.div
          ref={provided.innerRef}
          {...provided.draggableProps}
          {...provided.dragHandleProps}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 shadow-lg 
                     hover:shadow-purple-500/10 transition-all border border-gray-700"
        >
          <div className="flex justify-between items-start mb-6">
            <div>
              <h3 className="text-xl font-semibold text-white">
                {workspace.name}
              </h3>
              <p className="text-gray-400 mt-1 text-sm">
                {workspace.description}
              </p>
            </div>
            <div className="flex items-center space-x-2">
              <button
                onClick={handleRunAutomation}
                disabled={isRunning}
                className={`p-2 rounded-lg transition-all ${
                  isRunning 
                    ? 'bg-purple-500/20 text-purple-300' 
                    : 'bg-purple-500/10 hover:bg-purple-500/20 text-purple-400'
                }`}
              >
                {isRunning ? (
                  <ArrowPathIcon className="h-5 w-5 animate-spin" />
                ) : (
                  <SparklesIcon className="h-5 w-5" />
                )}
              </button>
              <button className="p-2 rounded-lg bg-gray-700/50 hover:bg-gray-700 
                               text-gray-400 transition-all">
                <EllipsisHorizontalIcon className="h-5 w-5" />
              </button>
            </div>
          </div>

          <div className="flex flex-wrap gap-2 mb-6">
            {workspace.apps.map((app) => (
              <div 
                key={app.id}
                className="flex items-center px-3 py-1.5 bg-gray-700/50 
                           rounded-full text-sm text-gray-300"
              >
                <img 
                  src={app.icon} 
                  alt={app.name} 
                  className="w-4 h-4 mr-2 rounded-full"
                />
                {app.name}
              </div>
            ))}
          </div>

          <div className="flex justify-between items-center text-sm">
            <div className="flex items-center text-gray-400">
              <ChartBarIcon className="h-4 w-4 mr-1.5" />
              <span className={workspace.stats.performance24h >= 0 ? 'text-green-400' : 'text-red-400'}>
                {workspace.stats.performance24h > 0 ? '+' : ''}
                {workspace.stats.performance24h}%
              </span>
              <span className="mx-2">•</span>
              <span>${workspace.stats.totalValue.toLocaleString()}</span>
            </div>
            <div className="text-gray-500">
              {workspace.automations} automations • Last active {workspace.lastActive}
            </div>
          </div>
        </motion.div>
      )}
    </Draggable>
  );
};
