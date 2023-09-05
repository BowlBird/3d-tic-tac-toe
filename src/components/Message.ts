export enum MessageType {
    ConnectionRequest,
    Confirmation
}

export type Message = {
    id: string;
    from: string;
    username: string;
    type: MessageType;
    content: string;
}

export function Message(from: string, username: string, type: MessageType, content: string): Message {
    return {
        id: MakeID(7),
        from: from,
        username: username,
        type: type,
        content: content
    }
}

function MakeID(length: number) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    let counter = 0;
    while (counter < length) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
        counter += 1;
    }
    return result;
}
