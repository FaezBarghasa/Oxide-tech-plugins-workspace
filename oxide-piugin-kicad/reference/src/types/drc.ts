import { Trace, Footprint } from './pcb';

export interface DesignCheckError {
  type: string;
  severity: 'error';
  trace?: Trace;
  footprints?: [Footprint, Footprint];
  minWidth?: number;
  minClearance?: number;
  actualDistance?: number;
}

export interface DesignCheckWarning {
  type: string;
  severity: 'warning';
  trace?: Trace;
  netName?: string;
  maxWidth?: number;
}

export interface DesignCheckReport {
  timestamp: Date;
  errorCount: number;
  warningCount: number;
  errors: DesignCheckError[];
  warnings: DesignCheckWarning[];
}
