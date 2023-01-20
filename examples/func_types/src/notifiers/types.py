from typing import TypeAlias
from kybra import blob, Func, Canister, method, Oneway, void

NotifierFunc: TypeAlias = Func(Oneway[[blob], void])


class Notifier(Canister):
    @method
    def get_notifier(self) -> NotifierFunc:
        ...
