import { invoke } from "@tauri-apps/api";

export enum MessageType {
    Confirmation,
    ConnectionRequest,
    ConnectionApproval,
    HeartbeatCall,
    HeartbeatResponse,
    LobbyClientDisconnectionNotification,
    LobbyHostDisconnectionNotification,
    LobbyInitialInformationRequest,
    LobbyInitialInformationResponse,
    LobbyInformationUpdate,
    GameStartNotification,
    GameStateUpdate,
    GameTurnAttempt,
}

export type Message = {
    id: string;
    from: string;
    username: string;
    type: MessageType;
    content: string;
}

export function createMessage(username: string, type: MessageType, content: string): Promise<Message> {
    return invoke('get_encrypted_ip_address').then( (ip: unknown) =>  {
        return {
            id: makeID(7),
            from: ip as string,
            username: username,
            type: type,
            content: content
        }
    });
}

export function messageFromString(message: string): Message {
    return JSON.parse(message) as Message
}

function makeID(length: number) {
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
