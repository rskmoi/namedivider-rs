from typing import List, Dict, Any, Optional, Literal
import requests
import json


class DividedName:
    def __init__(self, family: str, given: str, separator: str, score: float, algorithm: str):
        self.family = family
        self.given = given
        self.separator = separator
        self.score = score
        self.algorithm = algorithm
    
    def __repr__(self):
        return f"DividedName(family='{self.family}', given='{self.given}', score={self.score:.4f}, algorithm='{self.algorithm}')"


class NameDividerClient:
    def __init__(self, base_url: str = "http://localhost:8000"):
        self.base_url = base_url
    
    def divide(self, names: List[str], mode: Literal["basic", "gbdt"] = "basic") -> List[DividedName]:
        """
        Divide names using the specified mode.
        
        Args:
            names: List of names to divide
            mode: Division mode, either 'basic' or 'gbdt'
        
        Returns:
            List of DividedName objects
        
        Raises:
            requests.RequestException: If the request fails
            ValueError: If the response is invalid
        """
        payload = {"names": names, "mode": mode}
        
        response = requests.post(
            f"{self.base_url}/divide",
            headers={"Content-Type": "application/json"},
            json=payload
        )
        
        response.raise_for_status()
        
        data = response.json()
        
        if "divided_names" not in data:
            raise ValueError("Invalid response format")
        
        return [
            DividedName(
                family=item["family"],
                given=item["given"],
                separator=item["separator"],
                score=item["score"],
                algorithm=item["algorithm"]
            )
            for item in data["divided_names"]
        ]
    
    def divide_basic(self, names: List[str]) -> List[DividedName]:
        """Divide names using BasicNameDivider."""
        return self.divide(names, "basic")
    
    def divide_gbdt(self, names: List[str]) -> List[DividedName]:
        """Divide names using GBDTNameDivider."""
        return self.divide(names, "gbdt")