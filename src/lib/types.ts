export interface QueryEvent {
  id: string;
  status: 'STARTED' | 'SUCCESS' | 'ERROR';
  metadata: StartedMetadata | StringMetadata;
}

export interface QueryState {
  id: string;
  status: 'STARTED' | 'SUCCESS' | 'ERROR';
  metadata: StartedMetadata | StringMetadata;
}

export interface StartedMetadata {
  id: string;
  args: string[];
}

type StringMetadata = string;