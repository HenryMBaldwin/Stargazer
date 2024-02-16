export interface QueryEvent {
    id: string;
    status: 'STARTED' | 'SUCCESS' | 'ERROR';
    metadata: Record<string, string>; // Adjust the type as needed
  }
  
  export interface QueryState {
    id: string;
    status: 'STARTED' | 'SUCCESS' | 'ERROR';
    metadata: Record<string, string>; // Adjust the type as needed
  }
  