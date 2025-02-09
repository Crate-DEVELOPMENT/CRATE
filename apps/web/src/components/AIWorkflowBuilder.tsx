import { FC, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  SparklesIcon, 
  ArrowPathIcon,
  CheckCircleIcon,
  XMarkIcon
} from '@heroicons/react/24/outline';

interface AIAction {
  id: string;
  type: string;
  description: string;
  confidence: number;
  params?: Record<string, any>;
}

interface AIWorkflowBuilderProps {
  onActionSelect?: (action: AIAction) => void;
}

export const AIWorkflowBuilder: FC<AIWorkflowBuilderProps> = ({ onActionSelect }) => {
  const [prompt, setPrompt] = useState('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [suggestedActions, setSuggestedActions] = useState<AIAction[]>([]);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async () => {
    if (!prompt.trim()) return;

    setIsProcessing(true);
    setError(null);

    try {
      // Simulate AI processing
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      // Mock AI suggestions
      const mockActions: AIAction[] = [
        {
          id: '1',
          type: 'SWAP',
          description: 'Swap SOL to USDC when price reaches $80',
          confidence: 0.92,
          params: {
            tokenIn: 'SOL',
            tokenOut: 'USDC',
            priceThreshold: 80
          }
        },
        {
          id: '2',
          type: 'MONITOR',
          description:
