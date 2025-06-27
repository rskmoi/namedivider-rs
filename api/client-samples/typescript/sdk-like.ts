export interface DividedName {
  family: string;
  given: string;
  separator: string;
  score: number;
  algorithm: string;
}

export interface DivideRequest {
  names: string[];
  mode?: 'basic' | 'gbdt';
}

export interface DivideResponse {
  divided_names: DividedName[];
}

export class NameDividerClient {
  private baseUrl: string;

  constructor(baseUrl: string = 'http://localhost:8000') {
    this.baseUrl = baseUrl;
  }

  async divide(names: string[], mode: 'basic' | 'gbdt' = 'basic'): Promise<DividedName[]> {
    const request: DivideRequest = { names, mode };
    
    const response = await fetch(`${this.baseUrl}/divide`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json() as DivideResponse;
    return data.divided_names;
  }

  async divideBasic(names: string[]): Promise<DividedName[]> {
    return this.divide(names, 'basic');
  }

  async divideGbdt(names: string[]): Promise<DividedName[]> {
    return this.divide(names, 'gbdt');
  }
}