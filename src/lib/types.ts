export interface QueryEvent {
  id: string;
  timestamp: string;
  status: 'STARTED' | 'SUCCESS' | 'ERROR';
  metadata: StartedMetadata | SuccessMetadata | ErrorMetadata;
}

export interface QueryState {
  id: string; //log id
  timestamp: string; //timestamp of first query event
  status: 'STARTED' | 'SUCCESS' | 'ERROR'; //status of most recent query event
  events: QueryEvent[]; //all query events associated with this id
}

export interface StartedMetadata {
  id: string;
  args: string[];
}

export interface SuccessMetadata {
  message: string;
}

export interface ErrorMetadata {
  message: string;
}

export interface Database {
  id: string;
  name: string;
  selected: boolean;
}
