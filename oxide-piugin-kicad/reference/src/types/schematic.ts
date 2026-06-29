export interface Component {
  reference: string;
  value: string;
  type: string;
}

export interface Connection {
  source: string; // Component reference
  target: string; // Component reference
}

export interface Net {
  name: string;
  connections: Connection[];
}

export interface SchematicData {
  components: Component[];
  nets: Net[];
}
