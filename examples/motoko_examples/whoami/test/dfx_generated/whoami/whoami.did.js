export const idlFactory = ({ IDL }) => {
    return IDL.Service({
        argument: IDL.Func([], [IDL.Principal], ['query']),
        id: IDL.Func([], [IDL.Principal], []),
        id_quick: IDL.Func([], [IDL.Principal], ['query']),
        installer: IDL.Func([], [IDL.Principal], ['query']),
        whoami: IDL.Func([], [IDL.Principal], [])
    });
};
export const init = ({ IDL }) => {
    return [IDL.Principal];
};
