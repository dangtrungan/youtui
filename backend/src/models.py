from dataclasses import dataclass, field
from typing import Any


@dataclass
class IpcRequest:
    id: int
    method: str
    params: dict[str, Any] = field(default_factory=dict)


@dataclass
class IpcResponse:
    id: int
    result: Any = None
    error: dict[str, Any] | None = None


@dataclass
class IpcError:
    code: int
    message: str
